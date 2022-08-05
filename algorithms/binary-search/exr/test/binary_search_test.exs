defmodule BinarySearchTest do
  use ExUnit.Case
  doctest BinarySearch

  test "binary search empty list" do
    assert BinarySearch.search({}, 23) == :not_found
  end

  test "binary search one element in list" do
    assert BinarySearch.search({23}, 23) == {:ok, 0}
  end

  test "binary search one element not in list" do
    assert BinarySearch.search({23}, 32) == :not_found
  end

  test "binary search some elements in list" do
    assert BinarySearch.search({1, 9, 45, 63, 31, 70, 20, 100, 2}, 31) == {:ok, 4}
  end

  test "binary search some elements not in list" do
    assert BinarySearch.search({1, 9, 45, 63, 31, 70, 20, 100, 2}, 23) == :not_found
  end
end
