subshader "Jedi_static_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Jedi";
}

subshader "Jedi_static_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/rebelpilot_hand";
}

subshader "Jedi_static_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.4 0.4 0.4;
	materialSpecular 0 0 0;
	materialSpecularPower 12.5;
	texture "texture/face_bri1_h";
}
