import type { Type280 } from '../batch-01/file-0280.js';
export interface Type281 {
  id: 281;
  name: 'File281';
  next: Type280;
}

export function make281(): Type281 {
  return { id: 281, name: 'File281', next: null as unknown as Type280 };
}
