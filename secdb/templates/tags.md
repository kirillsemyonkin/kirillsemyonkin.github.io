{% if tags %}
<ul class="list">
{% for tag in tags %}
<a href="/secdb/{{ lang }}/tags/{{ tag }}">
<li>{{ tag | tag_title: lang }}</li>
</a>
{% endfor %}
</ul>
{% else %}
{{ "no_tags" | i18n: lang }}
{% endif %}
