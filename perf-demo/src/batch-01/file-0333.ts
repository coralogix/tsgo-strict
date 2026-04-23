import type { Type332 } from '../batch-01/file-0332.js';
export interface Type333 {
  id: 333;
  name: 'File333';
  next: Type332;
}

export function make333(): Type333 {
  return { id: 333, name: 'File333', next: null as unknown as Type332 };
}
