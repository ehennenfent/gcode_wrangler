from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates

from pydantic import BaseModel

class GcodeJob(BaseModel):
    image_path: str = "https://fakeimg.pl/800x600"
    comment: str = ""

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
