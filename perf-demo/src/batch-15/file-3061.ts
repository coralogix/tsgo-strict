import type { Type3060 } from '../batch-15/file-3060.js';
export interface Type3061 {
  id: 3061;
  name: 'File3061';
  next: Type3060;
}

export function make3061(): Type3061 {
  return { id: 3061, name: 'File3061', next: null as unknown as Type3060 };
}
