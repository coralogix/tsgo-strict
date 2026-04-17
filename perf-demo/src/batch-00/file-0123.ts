import type { Type122 } from '../batch-00/file-0122.js';
export interface Type123 {
  id: 123;
  name: 'File123';
  next: Type122;
}

export function make123(): Type123 {
  return { id: 123, name: 'File123', next: null as unknown as Type122 };
}
