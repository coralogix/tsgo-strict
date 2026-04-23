import type { Type1252 } from '../batch-06/file-1252.js';
export interface Type1253 {
  id: 1253;
  name: 'File1253';
  next: Type1252;
}

export function make1253(): Type1253 {
  return { id: 1253, name: 'File1253', next: null as unknown as Type1252 };
}
