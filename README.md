# Advent of Code 2022 - Rust

As every year, getting into the advent of code challenge.

As very year, I might not go through the end.

# How to Run
```bash
cargo run --bin day_01
```

Hey Folks, I'm designing a Schema for our Python based service with Strawberry-graphql.
So far so good it's quite simple to use it. Now I'm having trouble with the dynamic generation of class. I feel I'm hitting a wall so I'm wondering if someone might have something . Let me explain with a simple example.
Let's assume this structure.

```python
@strawberry.type
class Query:
    @strawberry.field
    def reporting() -> Reporting:
        return Reporting()

@strawberry.type
class Reporting:
   @strawberry.field
   def reportingA(filterA:List[str],filterB:List[str]) -> ReportingA:
        # Some processing
        return ReportingA(...)

   @strawberry.field
   def reportingB(filterB:List[str]) -> ReportingB:
        # Some processing
        return ReportingB(...)
```


