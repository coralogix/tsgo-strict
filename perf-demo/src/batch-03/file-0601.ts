import type { Type600 } from '../batch-03/file-0600.js';
export interface Type601 {
  id: 601;
  name: 'File601';
  next: Type600;
}

export function make601(): Type601 {
  return { id: 601, name: 'File601', next: null as unknown as Type600 };
}
