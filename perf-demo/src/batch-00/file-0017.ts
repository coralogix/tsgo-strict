import type { Type16 } from '../batch-00/file-0016.js';
export interface Type17 {
  id: 17;
  name: 'File17';
  next: Type16;
}

export function make17(): Type17 {
  return { id: 17, name: 'File17', next: null as unknown as Type16 };
}
