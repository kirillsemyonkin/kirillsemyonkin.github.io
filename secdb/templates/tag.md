{% if pages %}
<ul>
{% for page in pages %}
{%- if page.available_in_lang -%}
<a href="/secdb/{{ lang }}/{{ page.path }}">
<li>
{{ page.path | page_title: lang }}

{{ page.path | page_description: lang }}
</li>
</a>
{%- else -%}
<li>
{{ page.path | page_title: default_lang }}
{%- for lang in page.languages -%}
<a href="/secdb/{{ lang }}/{{ page.path }}">
<sup class="lang">({{ lang }})</sup>
</a>
{%- endfor %}

{{ page.path | page_description: default_lang }}
</li>
{%- endif -%}
{% endfor %}
</ul>
{% else %}
{{ "no_pages" | i18n: lang }}
{% endif %}
