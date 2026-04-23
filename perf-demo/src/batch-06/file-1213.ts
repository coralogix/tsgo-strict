import type { Type1212 } from '../batch-06/file-1212.js';
export interface Type1213 {
  id: 1213;
  name: 'File1213';
  next: Type1212;
}

export function make1213(): Type1213 {
  return { id: 1213, name: 'File1213', next: null as unknown as Type1212 };
}
