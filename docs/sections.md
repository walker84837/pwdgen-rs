# Password Sections

## Table of Contents

  - [Overview](#overview)
  - [Use Case](#use-case)
  - [How Sections Work](#how-sections-work)
  - [Examples](#examples)

## Overview

CryptWeave uses "sections" to organize passwords into distinct segments
separated by hyphens. Each section is six characters, so you can make your
password as long or short as you want.

## Use Case

Imagine you're trying to log in to a website or change your password, but the
website has a maximum password limit and the default 3 sections gives you a
password that's too long. You can change the number of sections to get a shorter
password, so you can still use the same program to generate a password, or you
may want a shorter password to make it easier to type and/or remember.

## How Sections Work

The number of sections is controlled by the `--sections` or `-s` flag. Each
section is made up of random characters and is separated by dashes. The default
is 3 sections.

### Examples

**Example 1: Default Sections**

``` console
$ cwv
ntsjbm-Vferfn-7dfnff
```

**Example 2: Custom Sections**

``` console
$ cwv --sections 4
viumeo-cycbne-cypiqk-it8kXu
```

**Example 3: Edge Case Handling**

``` console
$ cwv --sections 0
Error: Number of sections can't be zero.
```

As a password cannot logically be divided into null segments, CryptWeave will
abort if null segments are requested.

-----

This documentation is licensed in accordance with the terms of the [GNU Free
Documentation License, v1.3](LICENSE.md).
