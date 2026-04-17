import type { Type888 } from '../batch-04/file-0888.js';
export interface Type889 {
  id: 889;
  name: 'File889';
  next: Type888;
}

export function make889(): Type889 {
  return { id: 889, name: 'File889', next: null as unknown as Type888 };
}
