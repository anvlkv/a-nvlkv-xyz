from transformers import AutoModelWithLMHead, AutoTokenizer
from transformers import pipeline
from diffusers import KandinskyV22PriorPipeline, KandinskyV22Img2ImgPipeline
from tomllib import loads
from PIL import Image
import torch
import os

# tokenizer = AutoTokenizer.from_pretrained("mrm8488/t5-base-finetuned-summarize-news")
# model = AutoModelWithLMHead.from_pretrained("mrm8488/t5-base-finetuned-summarize-news")

cv_summarizer = pipeline("summarization", model="knkarthick/MEETING_SUMMARY")

def summarize(text, max_length=77):
  # input_ids = tokenizer.encode(text, return_tensors="pt", add_special_tokens=False)

  # generated_ids = model.generate(input_ids=input_ids, num_beams=2, max_length=max_length,  repetition_penalty=2.5, length_penalty=1.0, early_stopping=True)

  # preds = [tokenizer.decode(g, skip_special_tokens=True, clean_up_tokenization_spaces=True) for g in generated_ids]

  # return preds[0]
  return cv_summarizer(text, min_length=2, max_length=max_length)[0]["summary_text"]

classifier = pipeline("zero-shot-classification",model="sileod/deberta-v3-base-tasksource-nli")

candidate_labels = ['personal life',
                     'society and relationships', 
                     'mindfulness and yoga', 
                     'fashion and industry', 
                     'art and textiles', 
                     'software engineering']

def classification(text):
    results = classifier(text, candidate_labels)
    return results["labels"]

pipe_prior = KandinskyV22PriorPipeline.from_pretrained(
    "kandinsky-community/kandinsky-2-2-prior", torch_dtype=torch.float32
)

#7
weights = [0.3, 0.3, 0.4, 0.5]
def generate_embeds(summary, mood, character, img, negative_prompt):
  print("Generating embeds")
  print(summary)
  print(mood)
  print(character)
  summary = pipe_prior(summary, guidance_scale=4.0, num_inference_steps=3)
  mood = pipe_prior(mood, guidance_scale=4.0, num_inference_steps=3)
  character = pipe_prior(character, guidance_scale=4.0, num_inference_steps=3)
  return pipe_prior.interpolate([summary, mood, character, img], weights, negative_prompt=negative_prompt)
    

img_pipe = KandinskyV22Img2ImgPipeline.from_pretrained(
    "kandinsky-community/kandinsky-2-2-decoder", torch_dtype=torch.float32)
#12
def generate_image(image_embeds):
    print("Generating image")
    image = img_pipe(image_embeds, num_inference_steps=3, height=512,
        width=512)
    return image

def process_entry(character, mood, negative_prompt, text):
    labels = classification(text)
    summary = summarize(text, max_length=77)
    img = Image.open("./static/origins/" + labels[0] + ".jpg")
    embeds  = generate_embeds(summary, mood, character, img, negative_prompt)
    image = generate_image(embeds)
    return image
 

def process_dir(dir_name, out_dir):
    print("Processing dir")
    with open(dir_name + "/index.md", "r") as f:
        text = f.read();
    [__, config, text] = text.strip().split("+++");

    config = loads(config)
    character = f"pronouns: {','.join(config['pronouns'])}; features: {','.join(config['features'])};"
    mood = f"mood: {','.join(config['mood'])};"
    negative_prompt = ','.join(config['negative_prompt'])

    index_entry = process_entry(character, mood, negative_prompt, text)
    index_entry.save(out_dir + "/index.png")

    for f in os.listdir(dir_name):
        if f.endswith(".md") & f != "index.md":
            with open(dir_name + "/" + f, "r") as f:
                text = f.read();
            [__, config, text] = text.strip().split("+++");
            print("Processing file: " + f)

            config = loads(config)
            entry_mood = f"{mood}, years:  {','.join(config['period'])};"
            entry = process_entry(character, entry_mood, negative_prompt, text)
            entry.save(out_dir + "/" + f.replace(".md", ".png"))
            

    

process_dir("./content/bio", "./static/bio")