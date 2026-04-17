import type { Type1888 } from '../batch-09/file-1888.js';
export interface Type1889 {
  id: 1889;
  name: 'File1889';
  next: Type1888;
}

export function make1889(): Type1889 {
  return { id: 1889, name: 'File1889', next: null as unknown as Type1888 };
}
