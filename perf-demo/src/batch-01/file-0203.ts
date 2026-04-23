import type { Type202 } from '../batch-01/file-0202.js';
export interface Type203 {
  id: 203;
  name: 'File203';
  next: Type202;
}

export function make203(): Type203 {
  return { id: 203, name: 'File203', next: null as unknown as Type202 };
}
