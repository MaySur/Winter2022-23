# **RUST**

## 1.  Install: 

*[RUST LINK](https://www.rust-lang.org/tools/install)

***

## 2. Create Django Project:

```
django-admin startproject <project_name>
```

* manage.py – to be able to execute commands on the Django file
* setting.py – contains settings for the Django application
* urls.py – a table of contents for our application, which consists a number of different URLs or routes that can you can visit
***

## 3. Running web sever:
```
python manage.py runserver
```
* What it does: 
  * goes and actually runs this webs server
* b.	This is to  contain multiple applications under one web server, typically one web application has multiple applications running – Like for google for example has google search, google images, google maps all different apps under one server.


## 4. Create App:
```
python manage.py startapp <app_name>
``` 
* Creates a new directory with the app_name
* views.py lets us describe what the user sees when they visit a particular route.

## 5. To install app into webserver:
* Go into the project_name directory and into settings.py 
* b.	Scroll down to `INSTALLED_APPS[] ` and add `<app_name>`

## 6.	To edit the app:
* Go into views.py under the <app_name> directory 
* Look at – [This File](https://github.com/MaySur/Summer2022/blob/master/django/first/hello/urls.py)
* Then create urls.py under the same directory to specify when to use the views.py and edit the urls.py like this – file link
*	Then do under the urls.py under the <project_name> directory and edit it like this – [This File](https://github.com/MaySur/Summer2022/blob/master/django/first/first/urls.py)

*	Run the web server not the app
*	Go into the web address and edit the link to contain /hello tag at the end 

## 7.	In case of `TemplateDoesNotExsit` at <app_name> error:
* Go into views.py under the <project_name> directory 
* Then go into settings.py then find `TEMPLATES[]`
*	Then do into `DIRS:[]`and add the path to the Template Dir starting with r’<template_dir>’

## 8.	When using sessions run:
```
python manage.py migrate
``` 

## 9.	CSS Style Sheet/ static webpage:
* Create static directory within in the <app_name> directory then create a <app_name> directory inside the static directory
* Edit the `index.htm` with under `head` tag:
```
<link rel="stylesheet" href="{% static '<app_name>/styles.css' %}">
```  
* Copy the memory addres of the static with the <app_name> directory
* Go into settings.py under <project_name> directory find `TEMPLATES[]` then do into `DIRS:[]`and add the path to the Template Dir starting with r ’<static_dir>’





