<?php

declare(strict_types=1);

namespace NowPayments;

use Exception;

/**
 * NOWPayments API error exception.
 */
class NowPaymentsError extends Exception
{
    protected ?int $statusCode = null;
    protected ?string $apiCode = null;
    protected $response = null;

    public function __construct(
        string $message,
        ?int $statusCode = null,
        ?string $apiCode = null,
        $response = null
    ) {
        parent::__construct($message);
        $this->statusCode = $statusCode;
        $this->apiCode = $apiCode;
        $this->response = $response;
    }

    public function getStatusCode(): ?int
    {
        return $this->statusCode;
    }

    public function getErrorCode(): ?string
    {
        return $this->apiCode;
    }

    public function getResponse()
    {
        return $this->response;
    }

    /**
     * Developer-friendly string for logs.
     */
    public function __toString(): string
    {
        $parts = [$this->message];
        if ($this->statusCode !== null) {
            $parts[] = "(status: {$this->statusCode})";
        }
        if ($this->apiCode !== null) {
            $parts[] = "[{$this->apiCode}]";
        }
        return implode(' ', $parts);
    }
}
