import type { Type777 } from '../batch-03/file-0777.js';
export interface Type778 {
  id: 778;
  name: 'File778';
  next: Type777;
}

export function make778(): Type778 {
  return { id: 778, name: 'File778', next: null as unknown as Type777 };
}
