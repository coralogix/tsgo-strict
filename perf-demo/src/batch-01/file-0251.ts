import type { Type250 } from '../batch-01/file-0250.js';
export interface Type251 {
  id: 251;
  name: 'File251';
  next: Type250;
}

export function make251(): Type251 {
  return { id: 251, name: 'File251', next: null as unknown as Type250 };
}
