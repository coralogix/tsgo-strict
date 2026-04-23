import type { Type1020 } from '../batch-05/file-1020.js';
export interface Type1021 {
  id: 1021;
  name: 'File1021';
  next: Type1020;
}

export function make1021(): Type1021 {
  return { id: 1021, name: 'File1021', next: null as unknown as Type1020 };
}
