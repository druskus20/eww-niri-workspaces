# eww-niri-workspaces

A rust binary that outputs the relevant information from Niri's through
niri-ipc to be consumed by eww.


```clojure
(defwidget workspaces [monitor]
  (box :orientation "h" 
       :class "workspaces"
       :space-evenly false
    (for wsp in {"${workspaces.outputs[monitor].workspaces}"}
      (eventbox :cursor "pointer"
        (button :onclick "niri msg action focus-workspace ${wsp.index}"
          (box :class "workspace ${wsp.is_active ? 'active' : ''} ${arraylength(wsp.columns) == 0 ? 'empty' : ''}"
            (for col in "${wsp.columns}"
              (box :halign "center" :class "column ${col.has_focused_window ? 'focused' : ''}"
                (label :text "")))))))))
```

