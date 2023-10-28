+++
title="Contact"
description="Let's Connect and Create Together! I'm just one message away. Reach out with your inquiries, project ideas, or collaboration opportunities."
[extra.poly.aside]
content="My invitation to connect is very open-ended. Please let me know anything you find important to know beforehand about your project or communication preferences. <br/><br/> I'm fluent in English, Dutch, and Russian. Please express yourself freely in one of those languages, and I'll be of thousand ears."
+++

# Get in touch

## 🗣👂🏽<span style="display: inline-block;transform: scale(-1, 1);">🗣</span>

But first, let's connect via a form or through one of these links: <a href="https://www.linkedin.com/in/anvlkv" target="_blank">LinkedIn</a>, <a href="https://github.com/anvlkv" target="_blank">GitHub</a>, <a href="https://stackoverflow.com/users/1774187/anvlkv" target="_blank">Stack Overflow</a>,<a href="https://exercism.org/profiles/anvlkv" target="_blank">exercism</a>.


<form ata-netlify="true" method="POST" name="contact"
  action="/contact-success/">
  
  {{ field(name="name", required=true, label="Name", autocomplete="given-name" ) }}

  {{ field(type="email",name="email", required=true, label="Email", autocomplete="email" ) }}
  
  {{ field(type="textarea",name="Message", required=true, label="Message", autocomplete="off" ) }}

  {{ formButtons(submit="Send me a message") }}

</form>
