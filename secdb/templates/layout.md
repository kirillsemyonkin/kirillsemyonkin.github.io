<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{{ title }} | SecDB</title>
<meta property="og:title" content="{{ title }}">
<meta property="og:description" content="{{ description }}">
<link href="https://fonts.googleapis.com/css2?family=Inter:opsz,wght@14..32,200&display=swap"
      rel="stylesheet">
<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@200&display=swap">
<style>
a {
    text-decoration: none;
    color: #ff036c;
}
body {
    margin: 3em;
}
.description {
    font-size: 1.25em;
    margin-bottom: 1.5em;
}
hr {
    margin: 2em 0;
}
code {
    font-family: 'JetBrains Mono', monospace;
}
pre {
    padding: 1em;
    overflow-x: auto;
    border-radius: 1em;
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
}
ul {
    padding: 0;
}
li {
    list-style-type: none;
    margin: 0;
    padding: 0.75em 1.5em;
    background-color: #fff1;
    border-radius: 1em;
    margin-bottom: 0.5em;
}
li p {
    margin: 0;
    margin-bottom: 0.1em;
}
a>li {
    color: #fff;
}
.tag {
    background-color: #ff036c;
    color: #fff;
    padding: 0.25em 0.5em;
    border-radius: 0.5em;
    clip-path: polygon(0.5em 0%, 100% 0%, 100% 100%, 0.5em 100%, 0% 50%);
    margin-right: 0.5em;
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
</head>
<body>

<div class="breadcrumbs">
{% for part in page.path | subpaths -%}
{%- if part.name -%}
[{{ part.name }}](/secdb/{{ lang }}/{{ part.path }})
{% else %}
[SecDB](/secdb/{{ lang }})
{%- endif -%}
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
