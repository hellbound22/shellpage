# Shellpage

Quickstart your blog posts in Github Pages.
Write your posts in Markdown and render them in html with templates, javascript and css. Shellpage uses Tera for templates, but you don't even need to use them, if you don't want.

Check out [this repo](https://github.com/hellbound22/blog) to see the results

### Example usage
```
# Creates new post and launches editor
shellpage new-post "BlogPost" -o

# Converts the Markdown files to HTML and apllies templates 
shellpage publish "BlogPost" 

# Updates index.html with the full list of blog posts
shellpage update-index
```

### Example config file
```toml
# config.toml

_sign_name = "John"
repo_path = "/home/user/blog/"
md_storage = "/home/user/blog/md/"
html_storage = "/home/user/blog/html/"
template_path = "/home/user/blog/templates/*"
```

