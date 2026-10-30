```
status json:
{
  version: 1,
  drive: string,
  chunk_size: int,
  start_offset: {
    basis: 'start' | 'end',
    offset: int,
  },
  end_offset: {
    basis: 'start' | 'end',
    offset: int,
  },
  drive_size: int,
  current_offset: int,
  finished: boolean,
}
```
