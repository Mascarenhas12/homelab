defmodule Higia.Repo do
  use Ecto.Repo,
    otp_app: :higia,
    adapter: Ecto.Adapters.Postgres
end
