from transformers import pipeline, AutoTokenizer
from diffusers import StableDiffusionPipeline
import torch
import os


with open("./content/bio.md", "r") as f:
    text = f.read();

paragraphs = text.strip().split("+++")[2].split("\n\n");
classifier = pipeline("zero-shot-classification",model="sileod/deberta-v3-base-tasksource-nli")


model_name = "knkarthick/MEETING_SUMMARY"
tokenizer = AutoTokenizer.from_pretrained(model_name)
cv_summarizer = pipeline("summarization", model="knkarthick/MEETING_SUMMARY")

sketcher = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5", torch_dtype=torch.float32, device="cpu")
# if torch.cuda.is_available() == True:
#   sketcher.to("cuda")
# else:
#     commandline_args = os.environ.get('COMMANDLINE_ARGS', "--skip-torch-cuda-test --no-half")

# text = "one day I will see the world"
candidate_labels = ['personality', 'society', 'mindfulness', 'fashion', 'art', 'software']

def classification(text):
    results = classifier(text, candidate_labels)
    return results["labels"]

def save_image(image, path):
    with open(path, "wb") as f:
        f.write(image)


for i, paragraph in enumerate(paragraphs):
    if len(paragraph) < 1:
        continue
    

    labels = classification(paragraph)
    
    max_length = min(len(tokenizer.tokenize(paragraph)) - 1, 100)
    
    summary = cv_summarizer(paragraph, min_length=2, max_length=max_length)
    prompt = "focus on '" + labels[1] + "' and '" + labels[0] + "' create quick pencil sketch:" + summary[0]["summary_text"]
    print(prompt)
    sketch = sketcher(prompt)
    for y, img in enumerate(sketch.images):  
      img.save ("./static/bio/" + str(i) + '-' + str(y) + ".png")
    
    # print(paragraph)
    # print(best_classification(paragraph, candidate_labels))