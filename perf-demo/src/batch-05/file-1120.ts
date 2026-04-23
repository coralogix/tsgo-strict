import type { Type1119 } from '../batch-05/file-1119.js';
export interface Type1120 {
  id: 1120;
  name: 'File1120';
  next: Type1119;
}

export function make1120(): Type1120 {
  return { id: 1120, name: 'File1120', next: null as unknown as Type1119 };
}
