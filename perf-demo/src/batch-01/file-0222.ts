import type { Type221 } from '../batch-01/file-0221.js';
export interface Type222 {
  id: 222;
  name: 'File222';
  next: Type221;
}

export function make222(): Type222 {
  return { id: 222, name: 'File222', next: null as unknown as Type221 };
}
