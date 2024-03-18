# Cloning

Particularly when you are getting started, it's very easy to use `.clone()` as a crutch for data. You have a variable, you want to retain ownership, and the borrow checker is making life hard. So you clone it.

Cloning is fast---usually. When you call `clone()`, you are making a deep copy of the cloned object. So if it has a lot of data, all of that data has to be copied. As the data grows, clone becomes slower and slower.

Lots of calls to `clone()` is a *code smell*.

> Code smells are like underwear smells. If it smells too bad, consider changing it!

If ownership is confusing, and you need to share an object---maybe it should be borrowed? If borrowing is problematic (because lifetimes become uncertain)---maybe it should be an `Rc`? Cloning an Rc is almost instant (it increments an integer)---copying a large chunk of data can take a while!