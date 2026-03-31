export interface ApiError {
  error: string;
  error_type: string;
  status: number;
}

export class ApiException extends Error {
  constructor(
    message: string,
    public errorType: string,
    public status: number
  ) {
    super(message);
    this.name = 'ApiException';
  }
}

export async function handleApiResponse<T>(response: Response): Promise<T> {
  if (response.ok) {
    return response.json();
  }

  let errorMessage = response.statusText;
  let errorType = 'UnknownError';
  let status = response.status;

  try {
    const data: ApiError = await response.json();
    errorMessage = data.error;
    errorType = data.error_type;
    status = data.status;
  } catch {
    try {
      errorMessage = await response.text();
    } catch {
      errorMessage = response.statusText;
    }
  }

  throw new ApiException(errorMessage, errorType, status);
}
