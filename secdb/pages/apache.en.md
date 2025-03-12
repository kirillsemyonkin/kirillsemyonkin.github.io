Apache web server is a popular web server made for serving static or server-side generated content,
as well as proxying requests to other servers. It is made so that it can be easily configured
without any need to write code, but it can be extended with support for, most commonly, but not
limited to, PHP.

## In Debian-based systems

Installation with APT:

```bash
sudo apt install apache2
```

The configuration is available in the `/etc/apache2` directory.

```tree
├── apache2.conf
├── conf-available
│   ├── ...conf
├── conf-enabled
│   ├── ...conf -> ../conf-available/...conf
├── envvars
├── magic
├── mods-available
│   ├── ...load
├── mods-enabled
│   ├── ...load -> ../mods-available/...load
├── ports.conf
├── sites-available
│   ├── 000-default.conf
│   └── default-ssl.conf
└── sites-enabled
    └── 000-default.conf -> ../sites-available/000-default.conf
```

The main configuration file is `apache2.conf`. Additionally, configurations can be spread across
multiple `.conf` files. There is also a *modules system*, where all code installed separately as
`.so` libraries is loaded into Apache.

The configurations and modules are placed into two directories - `available` and `enabled`. You may
put incomplete or currently unneeded files into `available` and enable them later either by
symlinking to `enabled` or running corresponding commands.

### Configuration

Each option like `Name Value...` contained in the config is named *directive*. Multiple directives
can be combined into a single block, surrounded by XML-like tags `<Tag Options> ... </Tag>`.

Here are some of the `main` settings of the configuration file, used as defaults for all sites:

```conf
# Place where the server stores failed PHP scripts and such.
# Usual place for this is `/var/log/apache2/error.log`.
ErrorLog ${APACHE_LOG_DIR}/error.log

# This is how to tell Apache to not allow to serve files in anywhere but `/var/www`.
<Directory />
        Options FollowSymLinks
        # whether Options can be overridden by local .htaccess files
        AllowOverride None
        # deny all inside /
        Require all denied
</Directory>
<Directory /var/www/>
        # Indexes means there will be a directory listing if `index.*` files are not present.
        Options Indexes FollowSymLinks
        AllowOverride None
        # but allow all inside /var/www/
        Require all granted
</Directory>
```

### Serving a website

Create a new `.conf` file or edit default `000-default.conf` in the `sites-available` directory:

```conf
# Block where the site description will be placed.
# The address specified is the *listening address*. A `*` wildcard character specifies that any way
# to connect this site will be allowed, meanwhile any other address will make the connections
# accepted only if client tries to connect to the specified address.
# The port after the `:` character is usually either 80 (http) or 443 (https).
<VirtualHost *:80>
    ServerName www.example.com
</VirtualHost>
```

When done setting up a website, enable it with a command:

```bash
sudo a2ensite example.com
sudo systemctl restart apache2

# disable with
sudo a2dissite example.com
sudo systemctl restart apache2
```

### Rewrite engine

[Rewrite engine](https://httpd.apache.org/docs/2.4/mod/mod_rewrite.html) allows to rewrite URLs,
i.e. to change what URL is actually being served.

```bash
sudo a2enmod rewrite
sudo systemctl restart apache2

# disable with
sudo a2dismod rewrite
sudo systemctl restart apache2
```

Then add to your website or use a `.htaccess` file in the document root directory:

```conf
RewriteEngine on

# If the requested file is not a file, load /index.html instead (without redirecting)
RewriteCond %{REQUEST_FILENAME} !-f
RewriteRule ^/$ /index.html [L]

# If you want a redirect instead, use `mod_alias`:
Redirect permanent / /files
```

### File server

In your virtual host you can specify a directory to be served, and Apache will behave as a file
server:

```conf
<Directory /opt/fileserver>
    # List files
    Options Indexes FollowSymLinks

    # You can also add basic HTTP authorization
    AuthType Basic
    # Try to place the .htpasswd file in a non-public place
    AuthUserFile /etc/apache2/.htpasswd
    AuthName "Auth to view files"
    # User must be present in the .htpasswd file
    Require valid-user
</Directory>
```

For the auth, you need to create a `.htpasswd` file:

```bash
# -c to create new, without to append
htpasswd -c /etc/apache2/.htpasswd my-user
```

It will contain a line like `my-user:$hash$`.

### Enabling SSL

Installation:

```bash
sudo a2enmod ssl
sudo systemctl restart apache2

# disable with
sudo a2dismod ssl
sudo systemctl restart apache2
```

To setup your website with SSL you need two files - a certificate and a key. You can generate a
self-signed certificate with `openssl`:

```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout /etc/ssl/private/apache.key \
    -out /etc/ssl/certs/apache.crt
```

Next, add them to your virtual host:

```conf
SSLEngine on
SSLCertificateFile /etc/ssl/certs/apache.crt
SSLCertificateKeyFile /etc/ssl/private/apache.key
```

Also, here is a virtual host that redirects all requests to HTTPS:

```conf
<VirtualHost *:80>
    RewriteEngine on
    Redirect permanent / https://%{HTTP_HOST}
</VirtualHost>
```

### Enabling PHP

This example uses 8.2 version.

```bash
sudo apt install libapache2-mod-php
sudo a2enmod php8.2
sudo systemctl restart apache2

# disable with
sudo a2dismod php8.2
sudo systemctl restart apache2
```

After this all `.php` files will be preprocessed before serving in every `<?php ... ?>` block.

## In Arch-based systems

Installation with `pacman` from `extra` repository:

```bash
sudo pacman -S apache
```

Configuration is similar, except by default it is not made to have various commands - all happens in
an `/etc/httpd/conf/httpd.conf` file, or included separately via the `Include` directive.

```tree
├── conf
│   ├── extra
│   │   ├── ....conf
│   ├── httpd.conf
│   ├── magic
│   └── mime.types
└── modules -> /usr/lib/httpd/modules
```

Enabling modules is done via uncommenting `LoadModule` lines in the file:

```conf
#LoadModule rewrite_module modules/mod_rewrite.so
LoadModule php_module modules/libphp.so
AddHandler php-script .php
```

Before enabling some module you need to install it - for example, PHP:

```conf
sudo pacman -S php-apache
```
