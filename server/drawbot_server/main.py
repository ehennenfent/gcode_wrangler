import typing as t

from fastapi import BackgroundTasks, FastAPI, Request, Response
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates

from .models import GcodeJob, PostedJob

app = FastAPI()
templates = Jinja2Templates(directory="templates")

# Global state
primary_job: t.Optional[GcodeJob] = None
all_jobs: t.List[GcodeJob] = []
_session_key = "fakesessionkey"
ephemeral_keys = {}

app.mount("/static", StaticFiles(directory="static"), name="static")


@app.get("/", response_class=HTMLResponse)
async def read_item(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})


@app.get("/primary_job", response_class=HTMLResponse)
async def get_primary(request: Request):
    return templates.TemplateResponse("primary_job.html", {"request": request, "primary_job": primary_job})


@app.get("/job_list", response_class=HTMLResponse)
async def job_list(request: Request):
    return templates.TemplateResponse("job_list.html", {"request": request, "jobs": all_jobs})


@app.get("/progress", response_class=HTMLResponse)
async def get_progress(request: Request):
    return templates.TemplateResponse("progress.html", {"request": request, "primary_job": primary_job})


@app.post("/run_current")
async def start_background_task(response: Response, background_tasks: BackgroundTasks):
    response.headers["HX-Trigger"] = "start_job"
    background_tasks.add_task(primary_job.start_background_task)
    return {}


@app.post("/next")
async def next_job(response: Response):
    global primary_job
    response.headers["HX-Trigger"] = "next_job"
    primary_job = all_jobs.pop(0)
    return {}


@app.post("/authenticate")
async def authenticate(username: str, session_key: str):
    if session_key == _session_key:
        newkey = "ephemeral key"
        ephemeral_keys.setdefault(username, set()).add(newkey)
        return newkey


@app.post("/submit")
async def post_job(new_job: PostedJob, background_tasks: BackgroundTasks):
    global primary_job
    if new_job.ephemeral_key not in ephemeral_keys.get(new_job.username, set()):
        return
    processed = GcodeJob.from_posted(new_job)
    if primary_job is None:
        primary_job = processed
    else:
        all_jobs.append(processed)
    background_tasks.add_task(processed.analyze)
