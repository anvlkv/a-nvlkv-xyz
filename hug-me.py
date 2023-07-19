from transformers import pipeline, AutoTokenizer
from diffusers import KandinskyV22Img2ImgPipeline, KandinskyV22PriorPipeline
import torch
import os
from PIL import Image


with open("./content/bio.md", "r") as f:
    text = f.read();

paragraphs = text.strip().split("+++")[2].split("\n\n");
classifier = pipeline("zero-shot-classification",model="sileod/deberta-v3-base-tasksource-nli")


model_name = "knkarthick/MEETING_SUMMARY"
tokenizer = AutoTokenizer.from_pretrained(model_name)
cv_summarizer = pipeline("summarization", model="knkarthick/MEETING_SUMMARY")

# sketcher = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5", torch_dtype=torch.float32, device="cpu")
pipe_prior = KandinskyV22PriorPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-prior", torch_dtype=torch.float32)
pipe = KandinskyV22Img2ImgPipeline.from_pretrained("kandinsky-community/kandinsky-2-2-decoder", torch_dtype=torch.float32)


candidate_labels = ['personal life',
                     'society and relationships', 
                     'mindfulness and yoga', 
                     'fashion and industry', 
                     'art and textiles', 
                     'software engineering']

mood = "prosperity, fulfillment, high-end"

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
    
    max_length = min(len(tokenizer.tokenize(paragraph)) - 1, 60)
    
    summary = cv_summarizer(f"({labels[0]}): {paragraph}", min_length=2, max_length=max_length)
    prompt = f"mood: {mood}; topic: {labels[1]}; artwork: {summary[0]['summary_text']}"
    print(prompt)

    negative_prompt = "3D, photo-realistic, low quality, bad, error, amateur, ugly, unprofessional, low resolution, incoherent, inappropriate, unappealing, dull, waste, face error, cut, incomplete, naive, rough, outdated"

    image_embeds, negative_image_embeds = pipe_prior(prompt, guidance_scale=4.0, num_inference_steps=13).to_tuple()

    original_image = Image.open("./static/origins/" + labels[0] + ".jpg")

    out = pipe(
        image=original_image,
        image_embeds=image_embeds,
        negative_image_embeds=negative_image_embeds,
        height=1024,
        width=1024,
        num_inference_steps=32,
        strength=0.37,
    )

    
    out.images[0].save(f"./static/bio/{str(i)}.png")

    # sketch = sketcher(prompt)
    # for y, img in enumerate(sketch.images):  
    #   img.save ("./static/bio/" + str(i) + '-' + str(y) + ".png")
    
    # print(paragraph)
    # print(best_classification(paragraph, candidate_labels))