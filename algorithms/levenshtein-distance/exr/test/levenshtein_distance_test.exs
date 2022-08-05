defmodule LevenshteinDistanceTest do
  use ExUnit.Case
  doctest LevenshteinDistance

  test "both strings empty" do
    assert LevenshteinDistance.distance("", "") == 0
  end

  test "first string empty" do
    assert LevenshteinDistance.distance("", "a") == 1
  end

  test "second string empty" do
    assert LevenshteinDistance.distance("a", "") == 1
  end

  test "get distance" do
    assert LevenshteinDistance.distance("Hello", "Jello") == 1
  end
end
