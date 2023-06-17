from fastapi import FastAPI, Request, BackgroundTasks
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from time import sleep

from pydantic import BaseModel

class GcodeJob(BaseModel):
    image_path: str = "https://fakeimg.pl/800x600"
    comment: str = ""
    progress: int = 0

    @property
    def get_progress(self):
        return f"{self.progress}%"
    
    def start_background_task(self):
        self.progress = 0
        for _ in range(100):
            self.progress += 1
            sleep(1)

app = FastAPI()

app.mount("/static", StaticFiles(directory="static"), name="static")


templates = Jinja2Templates(directory="templates")


primary_job = GcodeJob()
all_jobs = [GcodeJob(comment="foobar"),
            GcodeJob(comment="barfoo"),
            GcodeJob(comment="A"*64)]

@app.get("/", response_class=HTMLResponse)
async def read_item(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})

@app.get("/primary_job", response_class=HTMLResponse)
async def get_progress(request: Request):
    return templates.TemplateResponse("primary_job.html", {"request": request, "primary_job": primary_job})

@app.get("/job_list", response_class=HTMLResponse)
async def get_progress(request: Request):
    return templates.TemplateResponse("job_list.html", {"request": request, "jobs": all_jobs})


@app.get("/progress", response_class=HTMLResponse)
async def get_progress(request: Request):
    return templates.TemplateResponse("progress.html", {"request": request, "primary_job": primary_job})

@app.post("/run_current")
async def start_background_task(background_tasks: BackgroundTasks):
    background_tasks.add_task(primary_job.start_background_task)
    return "Go"