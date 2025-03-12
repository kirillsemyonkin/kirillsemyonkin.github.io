## Commands

As you log into a Linux terminal, you are usually greeted with a *shell*. It allows you to execute
commands. The commands are usually in format `command [-flags] <argument>` (unless some particular
command does otherwise). Flags are usually combined (e.g. `-a -l` is the same as `-al`).

Commands will be run in the *current directory* - file directory, relative to which non-absolute
paths (e.g. `a/b/c`, as opposed to absolute `/a/b/c` which starts from the filesystem root) will be
resolved (for example, for a current directory `/root` the resolved path will be `/root/a/b/c`). You
can also usually refer to the current directory as `.`, as well as parent directory as `..`.

The most common commands that you can use in a shell of a terminal are:

```bash
# print the current working directory
$ pwd
/home/user

# change the current directory
$ cd <path>

# list files in the current directory
$ ls
$ ls -la  # `-l` long format, `-a` show hidden files
drwx------ 2 user user 4096 .
drwxr-xr-x 3 root root 4096 ..
-rw------- 1 user user  855 .bash_history
-rw-r--r-- 1 user user  220 .bash_logout
-rw-r--r-- 1 user user 3526 .bashrc
-rw-r--r-- 1 user user  807 .profile

# create a new directory
$ mkdir <path>

# remove a file or directory
$ rm <path>
$ rm -r <path>  # recursively remove a directory - all of its contents

# copy a file or directory
$ cp <source> <destination>

# move a file or directory
$ mv <source> <destination>

# print the contents of a file
$ cat <path>
$ cat  # read from standard input

# print a string
$ echo <string>
$ echo -n <string> # no newline at the end
```

Some commands output text. This text is said to be output to the *standard output* (*stdout*). It is
referred to by number `1` (this is its *file descriptor*). The text that you input after the program
launch is *standard input* (*stdin*, FD `0`). Errors are put into separate stream *standard error*
(*stderr*, FD `2`).

You can redirect the output of one command into input of another command using a *pipe `|`*
character:

```bash
# `echo` would output into its `stdout`, `cat` would read it as `stdin`, and then `cat` outputs into
# `stdout`, which your shell would read and output for you to see
echo test | cat
```

You can also redirect the output of one command to a file using the `>` character:

```bash
# file would be created or recreated automatically
echo Hello > test.txt

# do >> to write to the end of an existing file (append)
echo , world! >> test.txt
```

## Basic Linux file structure

Linux usually places files in the following structure:

```tree
/
│ # Program binaries are usually put into these places
├── bin -> usr/bin
├── lib -> usr/lib
├── lib64 -> usr/lib64
├── usr
│   ├── bin
│   │ # You can often find things like default configurations here
│   └── share
│ # Virtual files that allow you to write to and read from processes and devices
├── proc
│   │ # Information about current process (e.g. the shell) can be found out here
│   └── self
├── dev
│   ├── stderr # Same as writing to `/proc/self/fd/2`
│   ├── stdout # Same as writing to `/proc/self/fd/1`
│   ├── stdin # Same as reading from `/proc/self/fd/0`
│   ├── random # Reading gives infinite pseudo-random bytes
│   ├── null # Reading always ends (EOF), while writing ignores what you write
│   └── zero # You can infinitely read byte `0` from here
│ # Configuration files of programs are usually placed into `etc`
├── etc
│   ├── passwd # A list of users and maybe their passwords
│   └── shadow # A list of users passwords
│ # A user's home directory. For user `user`, they can refer to it with path `~`
├── home
│   └── user
├── root
│ # Some external file systems (e.g. a USB stick) is usually *mounted* into one of these
├── media
├── mnt
│ # All 
├── var
│   └── log
└── opt
```

## Users, Groups and passwords

In Linux there are users and groups of users to have specific permissions for files and other tasks.

To create a user you can use the `useradd <username>` command, or its sibling `adduser <username>`,
which will interactively ask you user info and do further automatic setup:

```bash
$ sudo adduser test
Adding user `test` ...
Adding new group `test` (1001) ...
Adding new user `test` (1001) with group `test (1001)` ...
Creating home directory `/home/test` ...
Copying files from `/etc/skel` ...
New password:
Retype new password:
passwd: password updated successfully
Changing the user information for test
Enter the new value, or press ENTER for the default
        Full Name []:
        Room Number []:
        Work Phone []:
        Home Phone []:
        Other []:
Is the information correct? [Y/n]
Adding new user `test` to supplemental / extra groups `users` ...
Adding user `test` to group `users` ...
```

You can also make groups with the `groupadd <groupname>` or `addgroup <groupname>`. Then, to add a
user to a group, you can use `usermod -a -G <groupname> <username>`, where `-a` means adding groups
instead of overwriting all, and `-G` allows you to list groups.

To change the password of a user, you can use `passwd <username>`.

The user with the most permissions is `root`. You can use `su <username>` to switch to that user,
i.e. `su root`, or `sudo <command>` to run a command as `root`.

## Permissions

Linux allows you to prohibit access to files for some users or groups. For this permissions exist,
and there simply only 5 kinds of permissions (the file's *mode*):

- `r` - read
- `w` - write
- `x` - execute
- `s` - sticky bit (`setuid`), that allows execution as owner
- `g` - `setgid`, that allows execution as group, usually for directories to inherit parent
        directory's group

This can also be represented with a bit field (e.g. `111` for `rwx`, `001` for `--x`), as well as a
number that represents the permission (e.g. `7` for `111` for `rwx`).

There are also separation of permissions into types of users:

- User (`U`) - permissions of the owner of the file.
- Group (`G`) - permissions of the group of the file.
- Other (`O`) / World (`W`) - permissions of other users.

Permissions can be changed on files with the following commands:

```bash
# set permissions of a file to U=7, G=7, O=7
chmod 777 my_file

# set group of a file
chgrp my_group my_file

# set owner of a file
chown my_user my_file
```

## Hostname

Each computer has an associated name, which lets it be identified on the network or in network login
systems. You can set the hostname in the following ways:

```bash
# using hostnamectl
sudo hostnamectl set-hostname <new-hostname>

# using hostname file
sudo echo <new-hostname> > /etc/hostname
# or in the Nano editor
sudo nano /etc/hostname

# after changing file, reboot
sudo reboot
# or, with systemd
sudo systemctl reboot
```
