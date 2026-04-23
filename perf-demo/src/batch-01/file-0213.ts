import type { Type212 } from '../batch-01/file-0212.js';
export interface Type213 {
  id: 213;
  name: 'File213';
  next: Type212;
}

export function make213(): Type213 {
  return { id: 213, name: 'File213', next: null as unknown as Type212 };
}
