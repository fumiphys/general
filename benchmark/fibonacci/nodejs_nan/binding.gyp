{
  "targets": [
  {
    "target_name" :"fibonacci",
    "sources" : ["fibonacci.cc"],
    "include_dirs": [
      "<!(node -e \"require('nan')\")"
        ]
  }
  ]
}
