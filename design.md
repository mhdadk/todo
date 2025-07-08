We describe how the To-do app will work at a high-level. We follow the structure
given [here](https://www.hellointerview.com/learn/system-design/in-a-hurry/delivery).

## Functional requirements

* The user should be able to add, modify, and delete a task.
* The user should be able to mark a task as "completed".
* The user should be able to see the tasks they have not yet completed.
* The user should not be able to configure the To-do app.

## Entities

* `TodoApp`: a `struct` that stores the list of tasks and is operated on depending on
user input.
* Tasks: a list of Tasks

## Functions

* `add`: create a new a task based on user input.
* `modify`: modify an existing task based on user input.
* `delete`: delete an existing task.
* `display`: show the tasks that are not yet completed to the user.