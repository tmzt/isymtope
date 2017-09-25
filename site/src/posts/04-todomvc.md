extends: default.liquid
title: TodoMVC
subtitle: The famous counter, staple of all reactive frameworks.
is_section: true
hide_title: false
---

A nearly-functional [TodoMVC](http://todomvc.com/) implementation is quite straight forward, thanks to our method pipeline syntax. As new SQL-inspired pipeline syntax is also coming which will make data manipulation in reducers even simpler.

Features will be added to this demo to match the standard implementation as they are added to the _isymtope_ language and compiler.

```javascript
use html;

store {
    let todos = [{text: "One", complete: false, id: 0}, {text: "Two", complete: true, id: 1}];
    let entry = "";

    todos {
        action add(entry) => state + (entry + {complete: false, id: (value.map(item.id).max(x) + 1)});
        action toggle_complete(id) => state | set complete = (!item.complete) where (item.id == id) |unique id;
    }

    entry {
        action clear => "";
    }
}

component todo_item (todo) {
    li (class={completed: todo.complete}) {
        div (class="view") {
            input(class="toggle", type="checkbox") bind todo.complete as complete
                change || { dispatch toggle_complete(id: todo.id); } {}
            label { (todo.text) }
            button(class="destroy") {}
        }
     }
}

component new_todo (entry) {
    input(class="new-todo", placeholder="What needs to be done?", autofocus="autofocus") bind entry
        enterkey || { dispatch add(entry: {text: entry}); } {}
}

link(rel="stylesheet", href="https://unpkg.com/todomvc-common@1.0.3/base.css", type="text/css") {}
link(rel="stylesheet", href="https://unpkg.com/todomvc-app-css@2.1.0/index.css", type="text/css") {}

section(class="todoapp") {
    header(class="header") {
        h1() { ("todos") }
        new_todo(get entry) {}
    }

    section(class="main") {
        input(class="toggle-all", id="toggle-all", type="checkbox") {}
        label(class="", for="toggle-all") { ("Mark all as complete") }
        ul(id="todo-list", class="todo-list") {
            todo_item (for todo in todos) {}
        }
    }
}
```

<a href="assets/demo/app-mvc.html" target="_blank">Open TodoMVC demo in new tab</a>.
