import type { Type1007 } from '../batch-05/file-1007.js';
export interface Type1008 {
  id: 1008;
  name: 'File1008';
  next: Type1007;
}

export function make1008(): Type1008 {
  return { id: 1008, name: 'File1008', next: null as unknown as Type1007 };
}
