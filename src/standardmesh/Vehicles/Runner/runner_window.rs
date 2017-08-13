subshader "runner_window_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      	transparent true;
	depthWrite true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 96.0;
	envmap true;
	texture "texture/vehicles/runner/window";
}