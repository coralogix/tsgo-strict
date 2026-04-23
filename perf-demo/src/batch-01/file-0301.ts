import type { Type300 } from '../batch-01/file-0300.js';
export interface Type301 {
  id: 301;
  name: 'File301';
  next: Type300;
}

export function make301(): Type301 {
  return { id: 301, name: 'File301', next: null as unknown as Type300 };
}
