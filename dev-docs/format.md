```
status json:
{
  version: 1,
  drive: string,
  chunk_size: int,
  start_offset: {
    location: 'start' | 'end',
    offset: int,
  },
  end_offset: {
    location: 'start' | 'end',
    offset: int,
  },
  finished: boolean,
}
```
