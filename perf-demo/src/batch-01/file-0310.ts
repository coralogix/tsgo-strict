import type { Type309 } from '../batch-01/file-0309.js';
export interface Type310 {
  id: 310;
  name: 'File310';
  next: Type309;
}

export function make310(): Type310 {
  return { id: 310, name: 'File310', next: null as unknown as Type309 };
}
