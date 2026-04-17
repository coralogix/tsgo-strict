import type { Type1440 } from '../batch-07/file-1440.js';
export interface Type1441 {
  id: 1441;
  name: 'File1441';
  next: Type1440;
}

export function make1441(): Type1441 {
  return { id: 1441, name: 'File1441', next: null as unknown as Type1440 };
}
