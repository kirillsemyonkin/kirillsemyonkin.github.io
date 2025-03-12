<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{{ title }} | SecDB</title>
<meta property="og:title" content="{{ title }}">
<meta property="og:description" content="{{ description }}">
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@200;900&display=swap"
      rel="stylesheet">
<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@200&display=swap">
<style>
a {
    text-decoration: none;
    color: #ff036c;
}
body {
    margin: 0 auto;
    max-width: 50em;
    padding: 3em;
}
.description {
    font-size: 1.25em;
    margin-bottom: 1.5em;
}
h1, h2, h3, h4, h5, h6 {
    transform: scaleY(0.85);
    margin: 3em 0 1em;
    cursor: pointer;
}
h1 .link, h2 .link, h3 .link, h4 .link, h5 .link, h6 .link {
    display: inline-block;
    opacity: 0;
    transition: opacity 250ms;
    transform: scaleX(0.85);
}
h1:hover .link, h2:hover .link, h3:hover .link, h4:hover .link, h5:hover .link, h6:hover .link {
    opacity: 0.5;
}
h1 .link:hover, h2 .link:hover, h3 .link:hover, h4 .link:hover, h5 .link:hover, h6 .link:hover {
    opacity: 1;
}
hr {
    margin: 2em 0;
}
code {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.9em;
    background-color: #222;
    border-radius: 5px;
    color: #a9b7c6;
}
a code {
    color: inherit;
}
pre {
    margin: 0;
    padding: 1em;
    overflow-x: auto;
    background-color: transparent !important;
}
.code {
    border-radius: 1em;
    background-color: #222;
    margin: 1em 0;
}
.code>.header {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.75em;
    background-color: #333;
    padding: 1em;
    border-radius: 1em 1em 0 0;
}
.code>.header span:last-of-type {
    float: right;
    cursor: pointer;
}
body, select {
    background-color: #111;
    font-size: inherit;
    color: #fff;
    font-weight: 200;
    font-family: 'Inter', sans-serif;
}
select {
    display: inline-block;
    border-radius: 0.5em;
    padding: calc(0.25em - 2.5px) 0.5em;
    margin-left: 0.5em;
}
.list {
    padding: 0;
}
.list li {
    list-style-type: none;
    margin: 0;
    padding: 0.75em 1.5em;
    background-color: #fff1;
    border-radius: 1em;
    margin-bottom: 0.5em;
}
.list li p {
    margin: 0;
    margin-bottom: 0.1em;
}
a>li {
    color: #fff;
}
.tag {
    display: inline-block;
    background-color: #ff036c;
    color: #fff;
    padding: 0.25em 0.5em;
    border-radius: 0.5em;
    clip-path: polygon(0.5em 0%, 100% 0%, 100% 100%, 0.5em 100%, 0% 50%);
    margin-right: 0.5em;
    margin-top: 0.5em;
}
.breadcrumbs {
    display: inline-block;
    background-color: #fff1;
    border-radius: 0.5em;
    padding: 0.25em 0.5em;
}
.breadcrumbs p {
    margin: 0;
}
.breadcrumbs *+*:before {
    content: '';
    display: inline-block;
    width: 0.5em;
    height: 0.75em;
    background-color: #fff7;
    clip-path: polygon(0 0, 100% 50%, 0 100%, 0% 86%, 75% 50%, 0% 14%);
    vertical-align: middle;
    top: -0.15em;
    position: relative;
    margin: 0 0.5em;
}
</style>
<script src="https://cdn.jsdelivr.net/npm/clipboard@2.0.11/dist/clipboard.min.js"></script>
<script>
document.addEventListener('DOMContentLoaded', function() {
new ClipboardJS('span[copy]', {
target: trigger => trigger.parentElement.parentElement.lastElementChild
});
new ClipboardJS(`h1, h2, h3, h4, h5, h6`, {
text: trigger => window.location.href =
window.location.origin + window.location.pathname + window.location.search + '#' + trigger.id
});
});
</script>
</head>
<body>

<div class="breadcrumbs">
{% for subpath in page.path | subpaths: lang %}
[{{ subpath.name }}](/secdb/{{ lang }}/{{ subpath.path }})
{%- endfor %}
</div>
<select onchange="window.location.href = `/secdb/${event.target.value}/{{ page.path }}`">
{% for l in page.languages -%}
<option {% if l | eq: lang %}selected{% endif %} value="{{ l }}">{{ l | lang_display }}</option>
{%- endfor %}
</select>

{% for tag in page.tags -%}
<a class="tag" href="/secdb/{{ lang }}/tags/{{ tag }}">{{ tag | tag_title: lang }}</a>
{%- endfor %}

---

# {{ title }}

{% if description -%}
<div class="description">
{{ description }}
</div>
{%- endif %}

{{ content }}

</body>
</html>
