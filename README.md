# ractor-pyo3-example

This is an example Python library for Ractor-Based actors. It provides a simple implementation of actors using Rust's Ractor library and PyO3 for Python bindings.

## Building Python Library

```bash
    pip install maturin
    cd actor-py
    maturin dev
```

## Running example in Python
```python
from actor_py import init_counter, increase_counter, get_counter

async def execute():
    cnt1 = await init_counter()
    cnt2 = await init_counter()
    await increase_counter(cnt1, 1)
    await increase_counter(cnt2, 2)
    print(await get_counter(cnt1))
    print(await get_counter(cnt2))

if __name__ == "__main__":
    import asyncio
    asyncio.run(execute())

```