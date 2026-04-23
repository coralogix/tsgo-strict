import type { Type1024 } from '../batch-05/file-1024.js';
export interface Type1025 {
  id: 1025;
  name: 'File1025';
  next: Type1024;
}

export function make1025(): Type1025 {
  return { id: 1025, name: 'File1025', next: null as unknown as Type1024 };
}
