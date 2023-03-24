---
layout: page
title: Passworst
subtitle: The Worst Password Manager
description: A password manager which requires no files
toc: true
---
## [Download for Windows x64](https://github.com/fr000gs/passworst/releases/latest/download/passrust.exe)

## [Standalone binary for Linux](https://github.com/fr000gs/passworst/releases/latest/download/passrust)

Forget to forgot password [^1]

## Usage

1. Enter your username in top field (along with the website name)
2. Enter your master password below
3. Copy the hash generated into the password

## How does it work?

1. Each combination of username, website and master password is hashed to make a UNIQUE password
2. Even if the password is leaked the person who gets the password cannot get your master password

## Advantages

1. You no longer have to click the "forgot password" button
2. You don't have to carry around a file unlike other password managers

#### Copyright (C) 2023 fr000gs

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the [GNU General Public License](https://www.gnu.org/licenses/gpl.html)
along with this program.  If not, see <https://www.gnu.org/licenses/>.

#### Credits: 
1. [Vim](www.vim.org) on [Arch](archlinux.org) with [YCM](https://github.com/ycm-core/YouCompleteMe)

[^1]: [Forgot password?](releases)
