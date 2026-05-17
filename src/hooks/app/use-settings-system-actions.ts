import { useCallback } from "react"
import { invoke } from "@tauri-apps/api/core"
import {
  getEnabledPluginIds,
  saveAutoUpdateInterval,
  saveGlobalShortcut,
  savePinOnTop,
  saveStartOnLogin,
  type AutoUpdateIntervalMinutes,
  type GlobalShortcut,
  type PluginSettings,
} from "@/lib/settings"

type UseSettingsSystemActionsArgs = {
  pluginSettings: PluginSettings | null
  setAutoUpdateInterval: (value: AutoUpdateIntervalMinutes) => void
  setAutoUpdateNextAt: (value: number | null) => void
  setGlobalShortcut: (value: GlobalShortcut) => void
  setStartOnLogin: (value: boolean) => void
  setPinOnTop: (value: boolean) => void
  applyStartOnLogin: (value: boolean) => Promise<void>
}

export function useSettingsSystemActions({
  pluginSettings,
  setAutoUpdateInterval,
  setAutoUpdateNextAt,
  setGlobalShortcut,
  setStartOnLogin,
  setPinOnTop,
  applyStartOnLogin,
}: UseSettingsSystemActionsArgs) {
  const handleAutoUpdateIntervalChange = useCallback((value: AutoUpdateIntervalMinutes) => {
    setAutoUpdateInterval(value)

    if (pluginSettings) {
      const enabledIds = getEnabledPluginIds(pluginSettings)
      if (enabledIds.length > 0) {
        setAutoUpdateNextAt(Date.now() + value * 60_000)
      } else {
        setAutoUpdateNextAt(null)
      }
    }

    void saveAutoUpdateInterval(value).catch((error) => {
      console.error("Failed to save auto-update interval:", error)
    })
  }, [pluginSettings, setAutoUpdateInterval, setAutoUpdateNextAt])

  const handleGlobalShortcutChange = useCallback((value: GlobalShortcut) => {
    setGlobalShortcut(value)
    void saveGlobalShortcut(value).catch((error) => {
      console.error("Failed to save global shortcut:", error)
    })
    invoke("update_global_shortcut", { shortcut: value }).catch((error) => {
      console.error("Failed to update global shortcut:", error)
    })
  }, [setGlobalShortcut])

  const handleStartOnLoginChange = useCallback((value: boolean) => {
    setStartOnLogin(value)
    void saveStartOnLogin(value).catch((error) => {
      console.error("Failed to save start on login:", error)
    })
    void applyStartOnLogin(value).catch((error) => {
      console.error("Failed to update start on login:", error)
    })
  }, [applyStartOnLogin, setStartOnLogin])

  const handlePinOnTopChange = useCallback((value: boolean) => {
    setPinOnTop(value)
    void savePinOnTop(value).catch((error) => {
      console.error("Failed to save pin on top:", error)
    })
    invoke("set_window_pin_on_top", { pinned: value }).catch((error) => {
      console.error("Failed to update pin on top:", error)
    })
  }, [setPinOnTop])

  return {
    handleAutoUpdateIntervalChange,
    handleGlobalShortcutChange,
    handleStartOnLoginChange,
    handlePinOnTopChange,
  }
}
