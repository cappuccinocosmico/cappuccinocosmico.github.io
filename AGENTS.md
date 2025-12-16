# Global Directives
- Be very honest. Tell me something I need to hear even if I don't want to hear it.
- Be proactive and flag issues before they become problems.
- Make sure to ask questions if the task is unclear, or you feel the instructions dont make sense as you are completing a task.


# Code Architecture Directives
- Write and architect code with a **Zero technical debt** policy. This means you should take the time to design and implement solutions correctly from the start. And if you see a feature that is designed badley, fix and rearchitect it as soon as possible, before building anything else on top of it.
- Every line code that you write makes the project harder to mantain. Whenever you are adding a new feature, if possible always try to modify existing code instead of adding new modules. Furthermore, be agressive about removing unused or dead code using git commits to make it easily revertible.

# Code Style Directives

- Assertions detect programmer errors. Unlike operating errors, which are expected and handled, assertions are for detecting errors in the logic of your program. The only correct way to handle corrupt/illogical code is to crash. Assertions downgrade catastrophic correctness bugs into liveness bugs. As such try and make sure the average function has a minimum of two assertions. If you encounter a codebase without them, add them in where it makes sense.

- Avoid comments whenever possible as they are often a sign of unclear code. Your goal should be to write code where anyone skim-reading it gets a clear understanding of what it's doing. Always use extremely clear variable names, and use simple control flow to make your code easier to understand.

- Use assertions as documentation. Assertions are supposed to give anyone reading your code an idea of what the expected behavior is, as well as the possible ways that it can fail. Always try to write code like this:
```rs
fn clamp(input: i32, low: i32, high: i32) -> i32 {
    assert!(low <= high, "clamp requires low <= high");
    let clamped_value = if input < low {
        low 
    } else if x > high {
        high 
    } else { 
        input 
    };
    assert!(clamped_value >= lo, "clamped value must not fall below low");
    assert!(clamped_value <= hi, "clamped value must not exceed high");
    clamped_value
}
```
and never like this:
```rs
fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
    // This function clamps the input x between the values
    // lo : represents the low value
    // hi : represents the high value

    if x < lo { lo } else if x > hi { hi } else { x }
    // Returns the clamped value
}
```

