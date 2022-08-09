def bits(n) do
  n
    |> Integer.digits(2)
    |> Enum.sum
end
